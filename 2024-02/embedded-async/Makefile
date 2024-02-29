IMAGES = qr.eps baremetal.pdf threads.pdf tasks.pdf

all: slides.pdf smallslides.png

images: ${IMAGES}

slides.pdf slides.pdfpc: slides.tex images pdfpcnotes.sty
	pdflatex $<
	grep 'may have changed' slides.log && rm $@ && make $@ || true
	sed -i -e 's/\\\\/\n/g' -e 's/\\par/\n\n/g' $@pc

qr.eps:
	qrencode -t eps https://github.com/RustVienna/meetup-history/tree/master/2024-02/embedded-async/ -o $@

pdfpcnotes.sty:
	wget -O$@ https://github.com/cebe/pdfpc-latex-notes/raw/master/pdfpcnotes.sty

smallslides.png: slides.pdf
	gm convert -density 300 $<[9] -resize 25%% $@

%.pdf: %.svg
	inkscape $< --export-filename $@

.phony: images
