require 'webrick'

server = WEBrick::HTTPServer.new :Port => 8000

server.mount_proc '/hello' do |req, res|
  res.body = '{"data":"world"}'
end

trap 'INT' do server.shutdown end

server.start