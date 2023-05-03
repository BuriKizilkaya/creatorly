use application::create::{
    service::{CreateProjectInput, CreateService},
    template_engine::TemplateEngine,
};
use clap::{command, Args, Subcommand};
use infrastructure::{
    configuration_loader::yaml_configuration_loader::YamlConfigurationLoader, file_system::FileSystem, folder_loader::git_files_loader::GitFileListLoader,
    folder_loader::local_file_loader::LocalFileListLoader, prompt::cli_prompt::CliPrompt, template_renderer::liquid_template_renderer::LiquidTemplateRenderer,
};
use std::sync::{Arc, Mutex};

#[derive(Args)]
pub struct Create {
    /// command
    #[command(subcommand)]
    sub_commands: CreateSubCommands,
}

#[derive(Subcommand)]
enum CreateSubCommands {
    /// Create a new project from a template
    Local(LocalCreate),

    /// Create a new project from a git repository
    Git(GitCreate),
}

#[derive(Args)]
struct LocalCreate {
    /// The path to the template
    #[arg(short, long)]
    template_path: String,

    /// The path to the destination path (it will be created if it does not exist)
    #[arg(short, long)]
    destination_path: String,
}

#[derive(Args)]
struct GitCreate {
    /// The path to the template
    #[arg(short, long)]
    remote_path: String,

    /// The name of the destination
    #[arg(short, long)]
    branch: String,

    /// The path to the destination path (it will be created if it does not exist)
    #[arg(short, long)]
    destination_path: String,
}

// parse create commands
pub fn parse_command(create: Create) {
    let configuration_loader = Arc::new(YamlConfigurationLoader {});
    let file_system = Arc::new(FileSystem {});
    let prompt = Arc::new(CliPrompt {});
    let liquid_template_renderer = Arc::new(Mutex::new(LiquidTemplateRenderer {}));
    let local_file_list_loader = Arc::new(LocalFileListLoader::default());
    let template_engine = Arc::new(TemplateEngine::new(liquid_template_renderer, file_system));

    match create.sub_commands {
        CreateSubCommands::Local(_local_create) => {
            let input: CreateProjectInput = CreateProjectInput {
                input_path: _local_create.template_path.trim_end_matches('/').to_string(),
                destination_path: _local_create.destination_path.trim_end_matches('/').to_string(),
            };
            let service: CreateService = CreateService::new(local_file_list_loader, configuration_loader, prompt, template_engine);
            service.create_project(input).unwrap();
        }
        CreateSubCommands::Git(_git_create) => {
            let git_file_list_loader = Arc::new(GitFileListLoader::new(local_file_list_loader, "/tmp/".to_string(), _git_create.branch));
            let input: CreateProjectInput = CreateProjectInput {
                input_path: _git_create.remote_path,
                destination_path: _git_create.destination_path.trim_end_matches('/').to_string(),
            };
            let service: CreateService = CreateService::new(git_file_list_loader, configuration_loader, prompt, template_engine);
            service.create_project(input).unwrap();
        }
    }
}
