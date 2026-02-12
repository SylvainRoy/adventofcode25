use std::collections::HashMap;
use std::{fs, usize};

#[derive(Clone, Copy, Debug)]
struct Result {
    fft_dac: usize,
    fft: usize,
    dac: usize,
    out: usize,
}

impl std::fmt::Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{fftdac:{}, fft:{}, dac:{}, out:{}}}",
            self.fft_dac, self.fft, self.dac, self.out
        )
    }
}

#[derive(Debug)]
struct Device {
    _name: String,
    cxns: Vec<usize>,
}

#[derive(Debug)]
struct Network {
    devices: Vec<Device>,
    _name_to_index: HashMap<String, usize>,
    you: usize,
    out: usize,
    fft: usize,
    dac: usize,
}

impl Network {
    fn from_file(string: &str) -> Self {
        let input = fs::read_to_string(string).expect("Failled to read input file.");
        let name_to_index = input
            .lines()
            .chain(std::iter::once("out: "))
            .map(|line| line[0..3].to_string())
            .enumerate()
            .map(|(index, name)| (name.clone(), index))
            .collect::<HashMap<String, usize>>();

        let devices = input
            .lines()
            .chain(std::iter::once("out: "))
            .map(|line| Device {
                _name: line[..3].to_string(),
                cxns: line[5..]
                    .trim()
                    .split_ascii_whitespace()
                    .map(|cxnname| name_to_index[cxnname])
                    .collect::<Vec<usize>>(),
            })
            .collect::<Vec<Device>>();
        let you = name_to_index["you"];
        let out = name_to_index["out"];
        let fft = name_to_index["fft"];
        let dac = name_to_index["dac"];

        Network {
            devices,
            _name_to_index: name_to_index,
            you,
            out,
            fft,
            dac,
        }
    }

    fn num_path_to_out(&self, idevice: usize) -> usize {
        if idevice == self.out {
            return 1;
        } else {
            self.devices[idevice]
                .cxns
                .iter()
                .map(|i| self.num_path_to_out(*i))
                .sum()
        }
    }

    fn num_path_to_out_with_fft_dac(
        &self,
        idevice: usize,
        depth: usize,
        cache: &mut HashMap<usize, Result>,
    ) -> Result {
        // println!(
        //     "{:pad$}{:3} ",
        //     "",
        //     self.devices[idevice]._name,
        //     pad = 4 * depth
        // );

        // End of recursion
        let res = if idevice == self.out {
            Result {
                fft_dac: 0,
                fft: 0,
                dac: 0,
                out: 1,
            }
        } else {
            // Recursion
            let f = if cache.contains_key(&idevice) {
                cache[&idevice]
            } else {
                self.devices[idevice]
                    .cxns
                    .iter()
                    .map(|cxnindex| self.num_path_to_out_with_fft_dac(*cxnindex, depth + 1, cache))
                    .fold(
                        Result {
                            fft_dac: 0,
                            fft: 0,
                            dac: 0,
                            out: 0,
                        },
                        |acc, x| Result {
                            fft_dac: acc.fft_dac + x.fft_dac,
                            fft: acc.fft + x.fft,
                            dac: acc.dac + x.dac,
                            out: acc.out + x.out,
                        },
                    )
            };

            // Combine result of recursion with current device.
            if idevice == self.fft {
                Result {
                    fft_dac: f.fft_dac + f.dac,
                    fft: f.fft + f.out,
                    dac: 0,
                    out: 0,
                }
            } else if idevice == self.dac {
                Result {
                    fft_dac: f.fft_dac + f.fft,
                    fft: 0,
                    dac: f.dac + f.out,
                    out: 0,
                }
            } else {
                Result {
                    fft_dac: f.fft_dac,
                    fft: f.fft,
                    dac: f.dac,
                    out: f.out,
                }
            }
        };

        // println!("{:pad$}{:3} ", "", res, pad = 4 * depth);
        cache.insert(idevice, res);
        res
    }
}

fn main() {
    let network = Network::from_file("./data/input.txt");
    println!("Part one: {}", network.num_path_to_out(network.you)); // 613

    let mut cache: HashMap<usize, Result> = HashMap::new();
    println!(
        "Part two: {}",
        network
            .num_path_to_out_with_fft_dac(network._name_to_index["svr"], 0, &mut cache)
            .fft_dac
    );
}
