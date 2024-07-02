# bash script to install factors_of to your terminal.
#
# REQUIRES:
# git: https://git-scm.com/downloads
# cargo: https://www.rust-lang.org/tools/install
DIR=$pwd

# download the scripts to a default location
echo "downloading files";
cd /usr/local/;
git clone https://github.com/vulbyte/factors_of-cli;
cd $pwd

# compile with cargo
echo "compiling scipt";
cd /usr/local/factors_of-cli/;
cargo build -r;
cd $pwd

# add an alias to the users ~/.zshrc 
echo 'adding "factors_of" to ~/.zshrc';
echo "alias factors_of='/usr/local/factors_of-cli/target/release/factors_of'" >> ~/.zshrc;
source ~/.zshrc;

echo "if there were were any errors, please submit an issue here: https://github.com/vulbyte/factors_of-cli/issues"
echo "if all was successful, then:"
# goodbye
# tool finished being added
echo "tool installed! enjoy!"
