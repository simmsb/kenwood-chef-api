# Kenwood cooking chef API replacement

This project replaces the fresco kitchenos recipe API (fresco/ instantconnect/
instantpot/ kenwood cooking chef) with an alternative which allows you to write
your own recipes.

Currently, this API acts as a MITM between the official api and the device. By
installing a custom root cert on the device and redirecting traffic to a device
running this software, we can intercept requests and responses in order to
inject our own recipes while still permitting access to the official recipes.

For a guide on rooting and setting up a custom root cert on the device, check
[ROOTING.md](./ROOTING.md).

 <img width="3592" height="7874" alt="image" src="https://github.com/user-attachments/assets/fad0ab1e-4d54-4c23-be2a-e6dc72bf4756" />

