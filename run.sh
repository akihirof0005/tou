cargo build --release
if [ -d "./pdb" ]; then
  ./target/release/tou
else
  echo "Dependency file not found..."
fi

