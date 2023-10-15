# Wambuis

Record and plot metrics of your laptop's battery.

## Quickstart

After cloning the repository, start collceting metrics by running:

```bash
$ cargo run
```

Plot the metrics in a graph:

```bash
$ gnuplot --persistent plut.gnuplot
```

[Plot with discharge rate](plot.svg)

## Increase battery life

`powertop` is an amazing tool to find out which processes and devices
consume drain your battery. However, it only shows a snaphot with data
from a given moment. It doesn't show metrics over time. 

`wambuis` writes the status of your battery to a csv file. `gnuplot` generates
a line graph of that csv file.

### Find hungry processes
Run `powertop`. The `Overview` page lists processes and their CPU usage. The
load is expressed in µs/s or ms/s. The smaller these numbers, the better. To reduce
the energy consumption of your laptop, consider killing processes with a high load.

Based on this list, I stopped containerd and docker on my private laptop.

### Find unused devices
Press `Tab` 3 times to open the `Devices` tab. This page lists the devices and
their relative usage. Disable devices you're not using. In my case, I disabled bluetooth
to reduce energy consumption of "Radio device: btusb".

### Tunables
Press `Tab` one more time to land on the `Tunables` tab. This page lists a series
of options that can reduce your laptop's energy use even further. You can manually
enable each setting or run `powertop --auto-tune`. Note that the settings are not persisted accross reboots.

# Resources

* [Kernel docs power supply class](https://www.kernel.org/doc/html/latest/power/power_supply_class.html)
* [Header file power_supply.h](https://github.com/torvalds/linux/blob/master/include/linux/power_supply.h)

# License

[MIT](LICENSE)
