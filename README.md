# Programmation efficace

![Build status](https://github.com/uvsq-pef/prog-efficace/actions/workflows/deploy.yml/badge.svg)

## Introduction
Ce cours s'inscrit dans le cursus de licence informatique de l'[UVSQ](https://www.uvsq.fr/) en 3ème année au semestre 5.
Il a plusieurs objectifs :
* faire découvrir les caractéristiques d'un langage statiquement typé et compilé,
* présenter les outils de développement associés (compilateur, build),
* sensibiliser aux difficultés liées à la gestion de la mémoire, à la gestion d'erreurs, aux I/O, au multi-threading (sans doute trop ambitieux de tout aborder),
* mettre en évidence les différences d'efficacité liées aux points précédents.

Le langage choisi pour ce cours est le langage [Rust](https://www.rust-lang.org/).

## Construction du cours
Pour construire le cours, les outils suivants doivent être installés sur le système.
* [`bundler`](https://bundler.io/)
* [`graphviz`](https://graphviz.org/) (pour les diagrammes)
* [`rake`](https://ruby.github.io/rake/)

### Installation des dépendances (gems)
```
$ bundle install
```

### Construction du cours (à la main)
```
$ # Construction du support de cours
$ for chapter in "intro" "devtools" "rust" "modeles-memoire" "energie"; do
    cp src/"$chapter"/figs/*.svg src/"$chapter"/figs/*.png html/figs/
  done
$ bundle exec asciidoctor -r asciidoctor-diagram -D html/ src/index.adoc
$ # Construction des slides
$ wget -qO- https://github.com/hakimel/reveal.js/archive/refs/tags/3.9.2.tar.gz | \
        tar --transform 's/^reveal.js-3.9.2/reveal.js/' -xz -C html
$ mkdir -p html/css && cp src/custom.css html/css
$ for chapter in "intro" "devtools" "rust" "modeles-memoire" "energie"; do
    bundle exec asciidoctor-revealjs -r asciidoctor-diagram -D html/ src/"$chapter"/"$chapter".adoc
  done
```

### Construction du cours (avec rake)
```
$ bundle exec rake
```

## Exercices
1. [Devine mon nombre !](https://github.com/uvsq-pef/td_devine_mon_nombre)
1. [Exercices choisis](https://github.com/uvsq-pef/td_exercices_choisis)
1. [Arbres binaires de recherche](https://github.com/uvsq-pef/td_abr)
1. [Optimisation de la multiplication de matrices](https://github.com/uvsq-pef/td_mult_mat)

### Projet
* [Gestionnaire de fichiers multimédias](https://github.com/uvsq-pef/medman)

## Références
* Cours de Stanford [CS 110L: Safety in Systems Programming](https://reberhardt.com/cs110l/) ([blog](https://reberhardt.com/blog/2020/10/05/designing-a-new-class-at-stanford-safety-in-systems-programming.html))
* Article dans Nature qui compare l'impact carbone de différents langages de programmation pour des codes d'astrophysique, [The Ecological Impact of High-performance Computing in Astrophysics](https://arxiv.org/pdf/2009.11295.pdf)
* Article dans SLE qui mesure l'efficacité énergétique de plusieurs langages de programmation, [Energy efficiency across Programming Languages](https://greenlab.di.uminho.pt/wp-content/uploads/2017/09/paperSLE.pdf)
* Cyrille Bonamy, Cédric Boudinet, Laurent Bourgès, Karin Dassas, Laurent Lefèvre, et al.. [Je code : les bonnes pratiques en éco-conception ...](https://hal.archives-ouvertes.fr/hal-03009741/). 2020. ⟨[hal-03009741](https://hal.archives-ouvertes.fr/hal-03009741/)
