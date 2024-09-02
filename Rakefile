require 'rake/clean'
# rake/clean définit CLEAN (Rake::FileList) pour les fichiers temporaires
# et CLOBBER (Rake::FileList) pour les fichiers de sortie

chapters = %w[intro devtools rust modeles-memoire energie conc] # %w définit un tableau de chaînes

REVEALJS_VERION = "5.1.0"

# Array : https://ruby-doc.org/core-trunk/Array.html
# Rake::FileList : http://ruby-doc.org/stdlib-trunk/libdoc/rake/rdoc/Rake/FileList.html
chapter_files = Rake::FileList[chapters.map{|c| "src/#{c}/#{c}.adoc"}]
slide_outfiles = Rake::FileList[chapters.map{|c| "html/#{c}.html"}]
fig_dirs = Rake::FileList[chapters.map{|c| "src/#{c}/figs"}]

# ajoute les fichiers html comme fichiers intermédiaire
CLEAN.include(slide_outfiles)
CLEAN << "html/index.html"
CLEAN << "html/figs"
# ajoute le répertoire html comme fichier de sortie
CLOBBER << "html"

desc "Génère la version livre et la version slides"
task :default => [:init_html, :generate_book, :generate_slides]
# :default est un symbole Ruby
# task <cible> => <dépendances>

desc "Initialise le répertoire de destination des fichiers html"
task :init_html => %w[html/reveal.js html/figs html/css]
directory "html/reveal.js" => "html" do |t|
    sh <<~HEREDOC
        wget -qO- https://github.com/hakimel/reveal.js/archive/#{REVEALJS_VERION}.tar.gz | \
        tar --transform 's/^reveal.js-#{REVEALJS_VERION}/reveal.js/' -xz -C #{t.source}/
    HEREDOC
end

desc "Initialise le répertoire des images"
directory "html/figs" => "html" do |t|
    mkdir_p t.name
    cp Dir["figs/*.png", "figs/*.svg"], t.name
    fig_dirs.each do |d|
        puts "fig dir #{d}"
        cp Dir["#{d}/*.png", "#{d}/*.svg"], t.name
    end
end

desc "Initialise le répertoire des feuilles de styles CSS"
directory "html/css" => %w[src/custom.css html] do |t|
    mkdir_p t.name
    cp t.source, t.name
end

# directory task pour créer le répertoire
directory "html"

desc "Génère la version livre du cours"
task :generate_book => [:init_html, "html/index.html"]
# file task
file "html/index.html" => %w[src/index.adoc] + chapter_files do |t|
    sh "asciidoctor -r asciidoctor-diagram -D html/ #{t.source}"
end

desc "Génère les slides"
task :generate_slides => [:init_html] + slide_outfiles

rule '.html' => ->(f){source_for_html(f)} do |t|
    chapter = t.name.ext('').sub(/^html\//, '')
    sh "asciidoctor-revealjs -r asciidoctor-diagram -D html/ src/#{chapter}/#{chapter}.adoc"
end

def source_for_html(html_file)
    chapter = html_file.ext('').sub(/^html\//, '')
    Rake::FileList["src/#{chapter}/*.adoc"]
end

