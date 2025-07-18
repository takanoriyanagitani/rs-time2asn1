#!/bin/sh

der2fq(){
	cat /dev/stdin |
		fq -d asn1_ber
}

der2jer(){
	cat /dev/stdin |
		xxd -ps |
		tr -d '\n' |
		python3 -m asn1tools \
			convert \
			-i der \
			-o jer \
			./basictime.asn \
			CompactTimeCoarse \
			- |
		jq -c
}

hour=12
minute=34

dostime16bits=$(( ${hour} << 11 | ${minute} << 5 ))

printf '%04x' ${dostime16bits} |
	xxd -r -ps |
	wazero \
		run \
		./dostime2asn1coarse.wasm |
	der2jer
