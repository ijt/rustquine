cargo run > /tmp/q.rs
if ! diff main.rs /tmp/q.rs; then
	echo FAILED
	exit 1
fi
echo PASS
