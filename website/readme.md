# Canon Collision Website

## Production Setup (Ubuntu/AWS)

```bash
sudo apt-get install nginx gcc libssl-dev libssh2-1-dev zlib1g-dev pkg-config
# Setup letsencrypt: https://certbot.eff.org
newuser ubuntu
su ubuntu
cd ~
curl https://sh.rustup.rs -sSf | sh # use default settings
echo "PATH=$HOME/.cargo/bin:$PATH" >> .profile
echo "ROCKET_ENV=prod" >> .profile
echo "TERM=xterm" >> .profile
git clone https://github.com/rukai/canon_collision_infra
cd canon_collision_infra/website
cp nginx.conf /etc/nginx.conf
```

## Run

```bash
su ubuntu
tmux
cd ~/canon_collision_infra/website
cargo run --release
```
