use std::io;
use std::path::Path;
use clap::{Parser, Subcommand};
use fake_type::Type;

/// Fake type
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// 伪装文件
    Fake {
        /// 伪装类型 目前支持 gz, mp4, txt
        #[clap(value_parser)]
        ftype: String,

        /// 文件路径
        #[clap(value_parser)]
        path: String,
    },
    /// 恢复文件
    Restore {
        /// 文件路径
        #[clap(value_parser)]
        path: String,
    }
}

fn main() -> Result<(), io::Error> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Fake { ftype, path } => {
            let sss = match ftype.as_str() {
                "gz" => {
                    Type {
                        suffix: "gz".to_string(),
                        bytes: vec![31, 139, 8, 0],
                    }
                },
                "mp4" => {
                    Type {
                        suffix: "mp4".to_string(),
                        bytes: vec![0, 0, 0, 24, 102, 116, 121, 112, 51, 103, 112, 53],
                    }
                },
                "txt" => {
                    Type {
                        suffix: "txt".to_string(),
                        bytes: b"Never gonna give you up.".to_vec(),
                    }
                }
                _ => {
                    println!("请输入正确的类型! 目前仅支持 gz,mp4,txt");
                    return Ok(());
                }
            };
            fake_type::fake(Path::new(path), fake_type::FLAG, sss).expect("伪装失败");
        },
        Commands::Restore {path} => {
            fake_type::restore(Path::new(path)).expect("恢复失败");
        }
    }
    Ok(())
}