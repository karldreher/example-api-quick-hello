# All times in seconds, except speed_download in bytes/s
echo "response_code,time_namelookup,time_connect,time_appconnect,time_pretransfer,time_redirect,time_starttransfer,speed_download,time_total"
curl -w "@curl-format.txt" -o "/dev/null" -s http://127.0.0.1:8000/hello
