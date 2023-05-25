all: slides.pdf

slides.pdf: slides.tex qr.eps riot.pdf rust-embedded.pdf
	pdflatex -shell-escape $<
	grep 'may have changed' slides.log && rm $@ && make $@ || true

riot.svg:
	wget https://github.com/RIOT-OS/riot-os.org/raw/master/assets/img/riot-logo.svg -O $@

rust-embedded.svg:
	wget https://rust-embedded.github.io/blog/ewg-logo-blue-white-on-transparent.svg -O $@

%.pdf: %.svg
	inkscape $< --export-filename $@

qr.eps:
	qrencode -t eps https://github.com/RustVienna/meetup-history/tree/master/2023-05/rust-on-riot/ -o $@
