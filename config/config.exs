import Config

config :breakfastbot,
  bot_username: "breakfastbot"

import_config "#{Mix.env()}.exs"
