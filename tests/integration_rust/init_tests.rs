use std::str::FromStr;

use pixi::cli;
use pixi_manifest::FeaturesExt;
use rattler_conda_types::{NamedChannelOrUrl, Platform, Version};

use crate::common::PixiControl;

#[tokio::test]
async fn init_creates_project_manifest() {
    // Run the init command
    let pixi = PixiControl::new().unwrap();
    pixi.init().await.unwrap();

    // There should be a loadable project manifest in the directory
    let workspace = pixi.workspace().unwrap();

    // Default configuration should be present in the file
    assert!(!workspace.display_name().is_empty());
    assert_eq!(
        workspace.display_name(),
        &pixi.workspace_path().file_stem().unwrap().to_string_lossy(),
        "project name should match the directory name"
    );
    assert_eq!(
        workspace
            .workspace
            .value
            .workspace
            .version
            .as_ref()
            .unwrap(),
        &Version::from_str("0.1.0").unwrap()
    );
}

/// Tests that when initializing an empty project with a custom channel it is
/// actually used.
#[tokio::test]
async fn specific_channel() {
    let pixi = PixiControl::new().unwrap();

    // Init with a custom channel
    pixi.init()
        .with_channel("random")
        .with_channel("foobar")
        .await
        .unwrap();

    // Load the workspace
    let workspace = pixi.workspace().unwrap();

    // The only channel should be the "random" channel
    let channels = Vec::from_iter(workspace.default_environment().channels());
    assert_eq!(
        channels,
        [
            &NamedChannelOrUrl::Name(String::from("random")),
            &NamedChannelOrUrl::Name(String::from("foobar")),
        ]
    )
}

// Test the initialization from an existing pyproject.toml file without the pixi information
#[tokio::test]
async fn init_from_existing_pyproject_toml() {
    let pixi = PixiControl::new().unwrap();

    // Copy the pyproject.toml file to the workspace directory
    let workspace_path = pixi.workspace_path();
    let pyproject_toml = workspace_path.join("pyproject.toml");
    let pyproject_toml_contents = include_str!("../data/pixi_tomls/pyproject_no_pixi.toml");
    fs_err::write(&pyproject_toml, pyproject_toml_contents).unwrap();

    // Init a new project
    pixi.init()
        .with_format(cli::init::ManifestFormat::Pyproject)
        .await
        .unwrap();

    // Check if the new manifest still contains all the same data as before
    assert!(
        pixi.manifest_contents()
            .unwrap()
            .contains(pyproject_toml_contents)
    );

    // Check if the new manifest is readable by pixi and contains the default values
    let workspace = pixi.workspace().unwrap();
    assert!(
        workspace
            .default_environment()
            .platforms()
            .contains(&Platform::current())
    );
}

// TODO: enable and fix this test when we fix the global config loading
// #[tokio::test]
// async fn default_pypi_config() {
//     let pixi = PixiControl::new().unwrap();
//     // Create new PyPI configuration
//     let index_url: Url = "https://pypi.org/simple".parse().unwrap();
//     let mut pypi_config = PyPIConfig::default();
//     pypi_config.index_url = Some(index_url.clone());
//     pypi_config.extra_index_urls = vec![index_url.clone()];
//     // pypi_config.keyring_provider =
// Some(pixi::config::KeyringProvider::Subprocess);     let mut config =
// Config::default();     config.pypi_config = pypi_config;
//     pixi.init().await.unwrap();

//     // Load the workspace
//     let project = pixi.project().unwrap();
//     let options = project.environment("default").unwrap().pypi_options();
//     assert_eq!(options.index_url, Some(index_url.clone()));
//     assert_eq!(options.extra_index_urls, Some(vec![index_url]));

//     assert_eq!(
//         project.config().pypi_config().keyring_provider,
//         Some(pixi::config::KeyringProvider::Subprocess)
//     );
// }
