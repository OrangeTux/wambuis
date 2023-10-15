# Data on the X-axis is time.
set xdata time

set format x "%H:%M"

# The first column of the csv contains a unix timestamp.
# The values must be interpreted as seconds.
set timefmt '%s'

# The left y-axis represents the SoC as a percentage.
set ylabel 'SoC (%))'

# The left y-axis ranges from 0% to 100%.
set yrange [0:100]
set ytics nomirror # dont show the tics on that side

set grid y
set grid x

# The right y-axis reprents the discharge power in Watts.
set y2tics
set y2label 'Power (W)'
# The right y-axis starts at 0 W.
set y2range [0:]

set xlabel 'Time'

# Configure separator between fields to be ','.
set datafile separator ','

# Plot columns 1 (x) and 6 (y) from battery.csv as a line graph. The line is titled "discharge rate".
plot 'battery.csv' using 1:6 with lines title "battery level", \
	'' using 1:5 with lines axis x1y2 title "discharge rate"
	# Plot columns 1 (x) and 5 (right y) from battery.csv as a line graph. 
	# The line is titled "battery level".
	
while (1) {
    replot
    pause 1
}
