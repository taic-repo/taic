set terminal pdfcairo enhanced size 12in,4in font "Arial,28"
set output 'taic-ipc.pdf'
set boxwidth 1

calus(a) = 1000000 / a
calms(a) = 1000 / a

# set noborder
set grid ytics
set xtics nomirror
set ytics nomirror
set xtics 20





datafile0 = 'stat/taic-uni.res'
datafile1 = 'stat/taic-uni-uint.res'
datafile2 = 'stat/taic-bi.res'
datafile3 = 'stat/taic-bi-uint.res'
datafile4 = 'stat/signal.res'
datafile5 = 'stat/pipe.res'
datafile6 = 'stat/eventfd.res'

set xrange [0: 200]
set xtics 50

set format y "%2.0f"
set ylabel 'Avarage Latency(ms)'
unset key
set yrange [0:4]
plot datafile4 using ($0+1):(calms($1))  w p lw 3 t "signal", \
     datafile5 using ($0+1):(calms($1))  w p lw 3 t "pipe", \
     datafile6 using ($0+1):(calms($1))  w p lw 3 t "eventfd"

set ylabel 'Avarage Latency({/Symbol \155}s)'
unset key
set key horizontal reverse Left top box samplen 1
set yrange [4:14]
replot datafile0 using ($0+1):(calus($1))  w p lw 3 t "taic-uni", \
     datafile1 using ($0+1):(calus($1))  w p lw 3 t "taic-uni-uint", \
     datafile2 using ($0+1):(calus($1))  w p lw 3 t "taic-bi", \
     datafile3 using ($0+1):(calus($1))  w p lw 3 t "taic-bi-uint"
     


