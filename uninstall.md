# 🧹 Uninstallation Guide

This guide explains how to remove the program, whether it was installed locally or globally.

## 1. Local Installation

If you ran the program locally (from its folder), simply delete the file.

### 🔧 Linux & 🖥️ Windows

Navigate to the folder where the program is located and delete it:

```bash
rm <program_name>
```

Or on Windows:

```cmd
del <program_name>.exe
```

---

## 2. Global Installation

If you installed the program globally, you'll need to remove it from the system's `PATH`.

### 🌐 Linux

If you moved the program to `/usr/bin`, remove it with:

```bash
sudo rm /usr/bin/<program_name>
```

### 🌐 Windows

If you placed the program in a folder included in your system's `PATH`:

1. Delete the executable from the folder (e.g., `C:\Windows\System32` or `C:\Programs\MyTools`):

```cmd
del C:\<path>\<program_name>.exe
```

---
