echo "Updating Cora..."

curl -L https://github.com/fusiontech21/Cora/releases/latest/download/cora -o /tmp/cora
sudo chmod +x /tmp/cora
sudo mv /tmp/cora /usr/local/bin/cora

echo "© Cora updated successfully!"