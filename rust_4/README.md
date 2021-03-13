Unofficial Umeå University
==========================

Welcome! This is the unofficial Umeå University Beamer Template. This style has been developed mainly following the graphical profile of Umeå University, with the expectation of some fonts (sans serif). You can find this [here](https://www.aurora.umu.se/stod-och-service/kommunikation/grafisk-profil/). 

You can start using the template right away using [Overleaf](<insert URL here>), or you can download the source code and work locally.

Note: for some reason, the RGB values specified in the manual do NOT render correctly in Beamer, so they have been redefined for this document using the high level chromo-optic deep neural quantistic technology provided by Microsoft Paint's color picker.

# Example Presentation

You can see the example presentation contained in this repository [here](https://v1.overleaf.com/latex/templates/umea-university-unofficial-beamer-theme/ptvmzxqzjhcn.pdf).

# Requirements

This theme requires `xcolor` and `tikz` packages. For this reason, you have to begin your document with:
```TeX
\documentclass[usenames,dvipsnames]{beamer}
```
This is **mandatory** due to compatibility issues between these two packages and `beamer`.

# Acknowledgements

The development of this theme was made possible by the [TeX community at Stack Exchange](http://tex.stackexchange.com/). There, I found lots of answers that saved me hours and headaches.

This theme contains items borrowed from Ratul Saha's Presento theme, available [here](https://github.com/RatulSaha/presento). In particular, `framecard` and `framepic` are based on the respective counterparts in Presento.