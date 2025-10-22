
if !(command -v git > /dev/null 2>&1) then
	echo "Missing git install!"
	exit 127
fi

if !(command -v git > /dev/null 2>&1) then
	echo "Missing cargo install!"
	exit 127
fi

if !(command -v git > /dev/null 2>&1) then
	echo "Missing npm install!"
	exit 127
fi

cargo build --release
npm i
npm run build
exit 0
