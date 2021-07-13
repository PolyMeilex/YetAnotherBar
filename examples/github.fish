set key "GITHUB-KEY"
set len (curl "https://api.github.com/notifications?access_token=$key" | jq length)

if test $len != 0
	echo $len | awk '{printf("%s",$0)}' 
end