help:
	cat Makefile

grammar.txt:
	wget https://raw.githubusercontent.com/dfinity/motoko/master/doc/modules/language-guide/examples/grammar.txt

clean:
	rm -f grammar.txt