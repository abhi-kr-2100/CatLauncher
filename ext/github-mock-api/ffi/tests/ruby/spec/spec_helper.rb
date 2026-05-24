$LOAD_PATH.unshift(File.expand_path("..", __dir__))
require "github_mock_api_ruby"

RSpec.configure do |config|
  config.expect_with :rspec do |expectations|
    expectations.include_chain_clauses_in_custom_matcher_descriptions = true
  end
end
