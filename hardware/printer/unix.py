import os

printer_name = "your_printer_name"
file_name = "your_file_name"

# $man lpr
os.system(f"lpr -P {printer_name} {file_name}")

# https://smallbusiness.chron.com/sending-things-printer-python-58655.html
# Note that "printer_name" represents the name of the printer you use on your system and will vary. "file_name.txt" is the name of the text file used for printing and will also vary.
