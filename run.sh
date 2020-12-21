files="./pdb/*"
for filepath in $files; do
  echo $filepath
  ./target/release/tou $filepath
done

