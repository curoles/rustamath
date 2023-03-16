BIN=target/debug
OUT=target/doc_book/html/image

mkdir -p $OUT

$BIN/rustamath-polynomial plot -f $OUT/poly1 -s=-10 -e 10 -d 8 -- 200 -5 -1 0.1

if [ -d "target/criterion" ]
then
  cp -r target/criterion target/doc_book/html/bench/criterion
fi