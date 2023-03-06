```
██████╗ ███████╗ █████╗  ██████╗████████╗  ██████╗      ██╗ ██████╗ █████╗ ███╗  ██╗
╚════██╗██╔════╝██╔══██╗██╔════╝╚══██╔══╝  ╚════██╗     ██║██╔════╝██╔══██╗████╗ ██║
  ███╔═╝█████╗  ███████║╚█████╗    ██║       ███╔═╝     ██║╚█████╗ ██║  ██║██╔██╗██║
██╔══╝  ██╔══╝  ██╔══██║ ╚═══██╗   ██║     ██╔══╝  ██╗  ██║ ╚═══██╗██║  ██║██║╚████║
███████╗██║     ██║  ██║██████╔╝   ██║     ███████╗╚█████╔╝██████╔╝╚█████╔╝██║ ╚███║
╚══════╝╚═╝     ╚═╝  ╚═╝╚═════╝    ╚═╝     ╚══════╝ ╚════╝ ╚═════╝  ╚════╝ ╚═╝  ╚══╝
Faster than a souped-up muscle car and more reliable than a family you can count on.

Convert csv to json in no time, with minimal ram/memory usage!
```

# Introduction by Dominic Torreto
I asked Dominic Torreto (ChatGPT) to introduce this tool and here is what he has got to say! 

> Welcome to 2 Fast 2 Json, the open-source program that's faster than a souped-up muscle car and more reliable than a family you can count on. Just like my crew and I, this program is all about speed and efficiency. It takes your CSV files and converts them to JSON in no time flat, making it easier for you to process your data and get back to what really matters.

2 Fast 2 Json is a simple program, that takes csv and pumps it out to json. As an incident responder sometimes we maybe provided terabytes of log data to analyse and sometimes this is in csv. Although csv can be ingested to many log platforms in native csv format. This capability may not be available to you as a responder. 

Although tools exist to convert csv to json across multiple languages. I was not happy that most of the them eats memory like a sunday afternoon snack and crushes if you give it something too large. I also found most of these tools spent abnormal amounts of time considering the simple operation that they needed to perform.  

In my tests I was able to get 2 Fast 2 Json does this conversion 60% faster than other rust programs, while almost 1000% faster than python based ones. This is mostly thanks to the [@alexkornitzer](https://github.com/alexkornitzer)'s advice and support.  The tool also buffer writes to disk ensuring the memory usage stays stable and is at about ~26 Mbs of memory/ram space. 

```
Usage: twojson [OPTIONS] <SOURCE_FILE> <OUTPUT_FILE>

Arguments:
  <SOURCE_FILE>  CSV file to operate on. i.e., ./processes/me.csv
  <OUTPUT_FILE>  JSON file path to save to. i.e., ./save/me/here.json

Options:
  -d <DELIMITER>      CSV Delimiter [default: ,]
  -t                  Tab delimited CSV. If provided, overrides the -d, --delimiter flag [default: false]
  -h, --help          Print help
  -V, --version       Print version
```
