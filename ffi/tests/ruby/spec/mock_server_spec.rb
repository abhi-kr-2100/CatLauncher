require "spec_helper"

RSpec.describe GitHubMockAPI::MockServer do

  after { @server&.stop }

  describe ".start" do
    it "starts a server and returns a MockServer instance" do
      @server = described_class.start
      expect(@server).to be_a(described_class)
    end

    it "returns a valid URI" do
      @server = described_class.start
      expect(@server.uri).to match(/\Ahttp:\/\/127\.0\.0\.1:\d+\z/)
    end
  end

  describe ".start_on" do
    it "starts a server on the given host and port" do
      @server = described_class.start_on("127.0.0.1", 0)
      expect(@server).to be_a(described_class)
      expect(@server.uri).to match(/\Ahttp:\/\/127\.0\.0\.1:\d+\z/)
    end

    it "raises ArgumentError for an invalid host" do
      expect { described_class.start_on("not-an-ip", 0) }.to raise_error(ArgumentError)
    end
  end

  describe "#stop" do
    it "stops the server without error" do
      @server = described_class.start
      expect { @server.stop }.not_to raise_error
    end

    it "can be called multiple times without error" do
      @server = described_class.start
      @server.stop
      expect { @server.stop }.not_to raise_error
    end
  end

  describe "#uri" do
    it "returns a String" do
      @server = described_class.start
      expect(@server.uri).to be_a(String)
    end

    it "returns an HTTP URL" do
      @server = described_class.start
      expect(@server.uri).to start_with("http://")
    end
  end

  context "with multiple servers" do
    it "starts multiple servers on different ports" do
      s1 = described_class.start
      @server = s1
      s2 = described_class.start
      expect(s1.uri).not_to eq(s2.uri)
    ensure
      s1&.stop
      s2&.stop
    end
  end
end
