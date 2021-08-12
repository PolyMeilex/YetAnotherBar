set key "GITHUB-KEY"
set len (curl --location --request GET 'https://api.github.com/notifications' \
--header "Authorization: token $key" | jq length)

if test $len != 0
	echo $len
end