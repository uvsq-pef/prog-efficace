#! /usr/bin/env Rscript

require(ggplot2)
require(reshape2)

rapl = read.table("rapl.csv", sep=",", header=T)
yoko = read.table("yoko.csv", sep=",", header=T)
yoko$T = yoko$T+6

total = dcast(rapl, T ~ ., fun.aggregate=sum )
names(total)[2] = "P"
total$S = "rapl-total"
summary(total)

data = rbind(yoko, rapl, total)

summary(data)

p = ggplot(data = data, aes(x = T, y = P, color=S, linetype=S)) + geom_line() + xlab("t (s)") + ylab("P (W)")

p = p + theme_bw() + theme(legend.position="top", legend.title=element_blank())

p = p + annotate(geom="text", x=21, y =60, label="nbody 1 thread")
p = p + annotate(geom="text", x=46, y =76, label="nbody 2 threads")
p = p + annotate(geom="text", x=67, y =100, label="nbody 4 threads")
p = p + annotate(geom="text", x=102, y =65, label="copie 100M éléments RAM")
p = p + annotate(geom="text", x=85, y =40, label="idle")
p = p + geom_segment(aes(x = 21, y = 30, xend = 21, yend = 56), color="black", arrow = arrow(length = unit(0.5, "cm")), show.legend = F)


ggsave(file="calib-yoko.svg", plot=p, width=10, height=8)

