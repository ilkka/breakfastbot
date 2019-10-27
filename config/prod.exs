config :logger,
  backends: [Timber.LoggerBackends.HTTP],

config :timber,
  api_key: get_env("TIMBER_IO_API_KEY"),
  source_id: get_env("TIMBER_IO_SOURCE_ID")
