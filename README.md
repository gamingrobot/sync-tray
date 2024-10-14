
# Linux tray app for Resilio Sync

## Install Resilio Sync

https://help.resilio.com/hc/en-us/articles/206178924-Installing-Sync-package-on-Linux

## Install dependencies

```
sudo dnf install gtk3-devel webkit2gtk4.1-devel
sudo dnf install gtk3 xdotool libappindicator-gtk3 #or libayatana-appindicator-gtk3
```

## Sync config

```
"webui" :
{
    "force_https": false,
    "listen" : "127.0.0.1:8888"
}
```


## Notes

Currently requires `WEBKIT_DISABLE_DMABUF_RENDERER=1` under Wayland

---
**This project is not affiliated or endorsed by Resilio**
