# Rooting the Kenwood Cooking Chef (KCL96)

## Overview

I recently bought a Kenwood cooking chef: it's a fancy stand mixer which
can also heat the bowl, which makes it pretty nice for one-pot meals
(it's also super good at caramelising onions.)

Its other notable feature is its touchscreen which is used to control
the mixer, and is also used to show recipes which for this machine are
structured as sequences of instructions interspersed with presets for
the machine.

This recipe following is actually a pretty neat idea to me, but
unfortunately there's no capability to program my own recipes (well
there is... the API used by the app has some endpoints for 'forking"
recipes, and these forked recipes can have their ingredients and steps
replaced, but super annoyingly the header image of the recipe cannot be
changed. Seemingly this is because the forking feature is intended for
translations or ingredient substitutions).

The only way to get my own recipes on there is going to involve
replacing the API server to some degree. At first I tried a MITM of the
SSL traffic, but the device is properly validating certs, so my only
option would be to get into the device.

## Breaking in

![The debug pins of the device, viewing with the PCB at the bottom, UART
TX is top left, UART RX is top
right.](https://github.com/user-attachments/assets/a74881a7-536b-4966-bd2b-ab779822eba5){width="80%"}

Thankfully, the device designers decided to leave a Mini-B USB port and
the UART pins exposed behind a press-fit cover under the device.

If the device is powered up while connected to a computer over USB, it
won't boot and will instead expose itself as 'SE blank 815'. I initially
went down a rabbit hole attempting to use this to dump the device before
I realised I should probably poke each of the pins on the debug header
with my scope to discover what they are.

By pure chance the first pin I probed ended up being the UART TX (it
idles at 3.3V and I could see occasional UART transmissions on the
scope). The second pin of the four I tried also was pulled to 3.3V, so I
guessed it was either UART RX or a 3.3V VCC. By luck it was the UART RX.

With the UART pins located, I connected them to a USB UART bridge using
some probe clips, and immediately was able to inspect the boot log.

    U-Boot SPL 2020.04-11.02.20-halo+gad7b74b415a (Mar 05 2021 - 07:05:56 +0000)\r\n
    DDRINFO: start DRAM init\r\n
    DDRINFO: DRAM rate 3200MTS\r\n
    PSM-fracpll_configure: audio_pll1_fdiv_ctl0=1331250\r\n
    PSM-fracpll_configure: audio_pll1_sscg_ctl=2147713090\r\n
    PSM-fracpll_configure: audio_pll2_fdiv_ctl0=1331250\r\n
    PSM-fracpll_configure: audio_pll2_sscg_ctl=2147713090\r\n
    PSM-fracpll_configure: video_pll1_fdiv_ctl0=405522\r\n
    PSM-fracpll_configure: video_pll1_sscg_ctl=2147516546\r\n
    PSM-fracpll_configure: dram_pll_fdiv_ctl0=819249\r\n
    PSM-fracpll_configure: dram_pll_sscg_ctl=2147614786\r\n
    PSM-fracpll_configure: audio_pll1_fdiv_ctl0=1331250\r\n
    PSM-fracpll_configure: audio_pll1_sscg_ctl=2147713090\r\n
    PSM-fracpll_configure: audio_pll2_fdiv_ctl0=1331250\r\n
    PSM-fracpll_configure: audio_pll2_sscg_ctl=2147713090\r\n
    PSM-fracpll_configure: video_pll1_fdiv_ctl0=405522\r\n
    PSM-fracpll_configure: video_pll1_sscg_ctl=2147516546\r\n
    PSM-fracpll_configure: dram_pll_fdiv_ctl0=819249\r\n
    PSM-fracpll_configure: dram_pll_sscg_ctl=2147614786\r\n
    PSM-fracpll_configure: audio_pll1_fdiv_ctl0=1331250\r\n
    PSM-fracpll_configure: audio_pll1_sscg_ctl=2147713090\r\n
    PSM-fracpll_configure: audio_pll2_fdiv_ctl0=1331250\r\n
    PSM-fracpll_configure: audio_pll2_sscg_ctl=2147713090\r\n
    PSM-fracpll_configure: video_pll1_fdiv_ctl0=405522\r\n
    PSM-fracpll_configure: video_pll1_sscg_ctl=2147516546\r\n
    PSM-fracpll_configure: dram_pll_fdiv_ctl0=819249\r\n
    PSM-fracpll_configure: dram_pll_sscg_ctl=2147614786\r\n
    DDRINFO:ddrphy calibration done\r\n
    DDRINFO: ddrmix config done\r\n
    Normal Boot\r\n
    Trying to boot from BOOTROM\r\n
    image offset 0x0, pagesize 0x200, ivt offset 0x0\r\n
    \r\n
    Authenticate image from DDR location 0x401fcdc0...\r\n
    NOTICE:  BL31: v2.4(release):lf-5.10.y-1.0.0-0-gba76d337e\r\n
    NOTICE:  BL31: Built : 08:27:48, Mar  1 2021\r\n
    \r\n
    \r\n
    U-Boot 2020.04-11.02.20-halo+gad7b74b415a (Mar 05 2021 - 07:05:56 +0000)\r\n
    \r\n
    CPU:   i.MX8MNano Dual rev1.0 1500 MHz (running at 1200 MHz)\r\n
    CPU:   Commercial temperature grade (0C to 95C) at 44C\r\n
    Reset cause: POR\r\n
    Model: NXP i.MX8MNano LPDDR4 EVK board\r\n
    DRAM:  2 GiB\r\n
    MMC:   FSL_SDHC: 2\r\n
    Loading Environment from MMC... OK\r\n

From this we can gather that this is a NXP i.MX8MNano LPDDR4 EVK board
device. From the later logs we can also see this is running the Weston
wayland compositor.

    OK Started Weston, a Wayland \xe2\x80\xa6mpositor, as a system service.\r\n
    OK Started Halo a53 autostart service.\r\n
    OK Started enable a53 logging to a file.\r\n
    OK Started enable esp32 logging.\r\n
    OK Started enable ip port settings.\r\n
    OK Started KitchenOS Agent.\r\n
    OK Started enable network manager logging.\r\n
    OK Started re-connect the esp\xe2\x80\xa6sconnection autostart service.\r\n
    OK Reached target Multi-User System.\r\n
    Starting Update UTMP about System Runlevel Changes...\r\n
    OK Finished Update UTMP about System Runlevel Changes.\r\n
    [   10.106913] kauditd_printk_skb: 25 callbacks suppressed\r\n
    \r\r\n
    NXP i.MX Release Distro 5.10-gatesgarth halo ttymxc1\r\n
    \r\n
    halo login:

We also get a login prompt :) I tried the usual combinations, but had no
luck. So I decided to dump the emmc.

### Enabling u-boot USB mass storage {#u-boot-usb}

By spamming enter while booting, u-boot will enter a debug shell. From
there we can run `ums 0 mmc 2` to make the emmc available as a usb mass
storage device.

    u-boot=> ums 0 mmc 2\r\n
    UMS: LUN 0, dev 2, hwpart 0, sector 0x438000, count 0x400000\r\n

I then quickly imaged the disk for inspection.

## Analysis

The device has two 2GB root partitions for A/B booting, and a data
partition which is mounted to `/data`.

After using binwalk to extract everything, I discovered this seems to be
a reasonably sane or standard IoT setup (though for some reason the
device uses an ESP32 to act as a Wifi adapter.) The device does some
'cloud provisioning" with AWS and talks to a MQTT server hosted there.
It also uses [RAUC](https://pengutronix.de/en/software/rauc.html) to
handle OTAs.

I proceeded to then get sidetracked with reverse engineering everything
interesting on the filesystem:

The device runs the following interesting systemd services:

A53.service

:   Handles drawing the GUI using [Embedded
    Wizard](https://embedded-wizard.de/), kicks off some other services
    (TODO: list these), handles AWS Cognito auth, fetches recipes from
    the Fresco API, talks to the Wifi ESP32 and issues commands over
    DBUS to (TODO: which).

KitchenOS Agent.service

:   (TODO: what does this actually do?)

aws-iot-proxy

:   Not a systemd service for some reason, but instead started by
    A53.service. (TODO: what does this do)

TODO: which service talks to the M7, document it the 'M7"
microcontroller that runs the rest of the hardware.

## Opening the door

I tried cracking the password hashes in `/etc/shadow` and poking around
the filesystem for passwords, but unfortunately this yielded nothing
(Though I did find the password used by their QC wifi network).

So after poking around on the filesystem dump long enough to be
satisfied there wasn't any filesystem tamper checks (Files on the
filesystem had varying dates and there were a few logfiles in the root
filesystems) I decided to simply replace the password hash in
`/etc/shadow`.

<figure>
<pre><code>TODO: original /etc/shadow
    </code></pre>
<figcaption><p>Original <code>/etc/shadow</code></p></figcaption>
</figure>

I simply replaced the hash with:

``` sh
λ openssl passwd -6 -salt fMdeOBXQ3zeBy649 root
$6$fMdeOBXQ3zeBy649$IEn/YjxZ/SHXoSgQTNMJhlxM7J6D5dUGQQbreQkPTU8MqeCu4yj2ynJRpYW5LjowqaNhXLPJp7Tq0yTfjmj3k0
```

Then I repeated the same procedure as [before](#u-boot-usb), but instead
of imaging the disk, I mounted both root partitions and replaced
`/etc/shadow` on each.

With that done, reboot the machine, and now `root:root` can be used to
login :)

My next step was to enable sshd, but it turns out it was already setup
as a systemd socket service... Listening on port `30303` 🙃

## Tomfoolery

With SSH access secured, I decided I would further this by setting up
tailscale on the device.

``` sh
cd /data/
curl -o tailscale.tgz  "https://pkgs.tailscale.com/stable/tailscale_1.90.9_arm64.tgz"
tar -xzf tailscale.tgz
TODO: mv command
TOOD: save systemd service
```

And of course, running fbdoom:

https://github.com/user-attachments/assets/8e346418-98b7-467b-9fd3-e3b810028f7f

## Custom API server

Next I wanted to MITM the API used by the device, to do that I first
created a CA and SSL cert which I can use on the device.

``` sh
# Create CA key and cert
openssl genrsa -out ca.key 2048
openssl req -new -x509 -days 3650 -key ca.key -out ca.crt -subj "/CN=My IoT Device Root CA/O=IoT Solutions/C=DE"

# Create SSL Cert
openssl genrsa -out server.key 2048
openssl req -new -key server.key -out server.csr -subj "/CN=amazonaws.com/O=IoT API Server/C=DE"
cat <<EOF > server.ext
authorityKeyIdentifier=keyid,issuer
basicConstraints=CA:FALSE
keyUsage = digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment
subjectAltName = @alt_names

[alt_names]
DNS.1 = amazonaws.com
DNS.2 = *.amazonaws.com
DNS.3 = fresco-kitchenos.com
DNS.4 = *.fresco-kitchenos.com
EOF

openssl x509 -req -in server.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out server.crt -days 3650 -sha256 -extfile server.ext
```

The CA cert can then be installed on the device:

First, on your computer:

``` sh
scp ca.crt root@halo:/usr/share/ca-certificates/ca-custom.crt
```

And then on the mixer:

``` sh
echo "ca-custom.crt" >> /etc/ca-certificates.conf
update-ca-certificates --fresh
```

I then added entries to `/etc/hosts` for `api.fresco-kitchenos.com` and
`media.fresco-kitchenos.com`. (I'd identified these by sniffing traffic
and REing the mobile app and binaries on the device.)

## Next steps

The api and UI included in this repo can then be used.
