// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use log::info;
use service::{IdentifyVariant, self};
use sc_cli::{SubstrateCli, Result, RuntimeVersion, Role};
use crate::cli::{Cli, Subcommand};

fn get_exec_name() -> Option<String> {
	std::env::current_exe()
		.ok()
		.and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
		.and_then(|s| s.into_string().ok())
}

impl SubstrateCli for Cli {
	fn impl_name() -> String { "PgRocks Pirl".into() }

	fn impl_version() -> String { env!("SUBSTRATE_CLI_IMPL_VERSION").into() }

	fn description() -> String { env!("CARGO_PKG_DESCRIPTION").into() }

	fn author() -> String { env!("CARGO_PKG_AUTHORS").into() }

	fn support_url() -> String { "https://github.com/kl3eo/pirl2/issues/new".into() }

	fn copyright_start_year() -> i32 { 2017 }

	fn executable_name() -> String { "pirl".into() }

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		let id = if id == "" {
			let n = get_exec_name().unwrap_or_default();
			["pirl", "kusama", "westend", "rococo"].iter()
				.cloned()
				.find(|&chain| n.starts_with(chain))
				.unwrap_or("pirl")
		} else { id };
		Ok(match id {
			"polkadot-dev" | "dev" => Box::new(service::chain_spec::polkadot_development_config()?),
			"polkadot-local" => Box::new(service::chain_spec::polkadot_local_testnet_config()?),
			"polkadot-staging" => Box::new(service::chain_spec::polkadot_staging_testnet_config()?),
			"kusama-dev" => Box::new(service::chain_spec::kusama_development_config()?),
			"kusama-local" => Box::new(service::chain_spec::kusama_local_testnet_config()?),
			"kusama-staging" => Box::new(service::chain_spec::kusama_staging_testnet_config()?),
			"pirl" => Box::new(service::chain_spec::pirl_config()?),
			"westend" => Box::new(service::chain_spec::westend_config()?),
			"kusama" => Box::new(service::chain_spec::kusama_config()?),
			"westend-dev" => Box::new(service::chain_spec::westend_development_config()?),
			"westend-local" => Box::new(service::chain_spec::westend_local_testnet_config()?),
			"westend-staging" => Box::new(service::chain_spec::westend_staging_testnet_config()?),
			"rococo-staging" => Box::new(service::chain_spec::rococo_staging_testnet_config()?),
			"rococo-local" => Box::new(service::chain_spec::rococo_local_testnet_config()?),
			"rococo" => Box::new(service::chain_spec::rococo_config()?),
			path => {
				let path = std::path::PathBuf::from(path);

				let starts_with = |prefix: &str| {
					path.file_name().map(|f| f.to_str().map(|s| s.starts_with(&prefix))).flatten().unwrap_or(false)
				};

				// When `force_*` is given or the file name starts with the name of one of the known chains,
				// we use the chain spec for the specific chain.
				if self.run.force_rococo || starts_with("rococo") {
					Box::new(service::RococoChainSpec::from_json_file(path)?)
				} else if self.run.force_kusama || starts_with("kusama") {
					Box::new(service::KusamaChainSpec::from_json_file(path)?)
				} else if self.run.force_westend || starts_with("westend") {
					Box::new(service::WestendChainSpec::from_json_file(path)?)
				} else {
					Box::new(service::PolkadotChainSpec::from_json_file(path)?)
				}
			},
		})
	}

	fn native_runtime_version(spec: &Box<dyn service::ChainSpec>) -> &'static RuntimeVersion {
		if spec.is_kusama() {
			&service::kusama_runtime::VERSION
		} else if spec.is_westend() {
			&service::westend_runtime::VERSION
		} else if spec.is_rococo() {
			&service::rococo_runtime::VERSION
		} else {
			&service::pirl_runtime::VERSION
		}
	}
}

fn set_default_ss58_version(spec: &Box<dyn service::ChainSpec>) {
	use sp_core::crypto::Ss58AddressFormat;

	let ss58_version = if spec.is_kusama() {
		Ss58AddressFormat::KusamaAccount
	} else if spec.is_westend() {
		Ss58AddressFormat::SubstrateAccount
	} else {
		Ss58AddressFormat::PolkadotAccount
	};

	sp_core::crypto::set_default_ss58_version(ss58_version);
}

/// Parses polkadot specific CLI arguments and run the service.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		None => {
			let runner = cli.create_runner(&cli.run.base)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			let authority_discovery_disabled = cli.run.authority_discovery_disabled;
			let grandpa_pause = if cli.run.grandpa_pause.is_empty() {
				None
			} else {
				Some((cli.run.grandpa_pause[0], cli.run.grandpa_pause[1]))
			};

			if chain_spec.is_kusama() {
				info!("----------------------------");
				info!("This chain is not in any way");
				info!("      endorsed by the       ");
				info!("     KUSAMA FOUNDATION      ");
				info!("----------------------------");
			}

			runner.run_node_until_exit(|config| async move {
				let role = config.role.clone();

				match role {
					Role::Light => service::build_light(config).map(|(task_manager, _)| task_manager),
					_ => service::build_full(
						config,
						authority_discovery_disabled,
						grandpa_pause,
					).map(|full| full.task_manager),
				}
			})
		},
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			runner.async_run(|mut config| {
				let (client, _, import_queue, task_manager) = service::new_chain_ops(&mut config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			runner.async_run(|mut config| {
				let (client, _, _, task_manager) = service::new_chain_ops(&mut config)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			runner.async_run(|mut config| {
				let (client, _, _, task_manager) = service::new_chain_ops(&mut config)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			runner.async_run(|mut config| {
				let (client, _, import_queue, task_manager) = service::new_chain_ops(&mut config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			runner.async_run(|mut config| {
				let (client, backend, _, task_manager) = service::new_chain_ops(&mut config)?;
				Ok((cmd.run(client, backend), task_manager))
			})
		},
		Some(Subcommand::ValidationWorker(cmd)) => {
			let _ = sc_cli::init_logger("", sc_tracing::TracingReceiver::Log, None);

			if cfg!(feature = "browser") {
				Err(sc_cli::Error::Input("Cannot run validation worker in browser".into()))
			} else {
				#[cfg(all(not(feature = "browser"), not(feature = "service-rewr")))]
				service::run_validation_worker(&cmd.mem_id)?;
				Ok(())
			}
		},
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			runner.sync_run(|config| {
				cmd.run::<service::kusama_runtime::Block, service::KusamaExecutor>(config)
			})
		},
	}
}
