# Build file .exe trên Windows

GPUI dùng DirectX trên Windows và cần `fxc.exe` (trình biên dịch shader trong
Windows SDK) lúc build, nên **bắt buộc build trên máy Windows** — không
cross-compile từ Linux/macOS được.

## Chuẩn bị (làm 1 lần)

1. Cài **Rust**: https://rustup.rs (chọn mặc định `x86_64-pc-windows-msvc`)
2. Cài **Visual Studio Build Tools** kèm:
   - "Desktop development with C++"
   - **Windows 10/11 SDK** (chứa `fxc.exe`)

## Build

```powershell
cd todo-gpui
cargo build --release
```

File kết quả: **`target\release\todo-gpui.exe`** — chạy độc lập, không cần
cài thêm gì (DirectX có sẵn trong Windows).

Dữ liệu todo lưu tại `%USERPROFILE%\.todo-gpui.json`.

## Hoặc: build tự động bằng GitHub Actions

Repo đã có sẵn workflow `.github/workflows/build-windows.yml`:

1. Push code lên GitHub
2. Vào tab **Actions** → chọn run mới nhất
3. Tải artifact **todo-gpui-windows** (chứa `todo-gpui.exe`)

Mẹo: nếu `fxc.exe` không nằm trong PATH, đặt biến môi trường
`GPUI_FXC_PATH` trỏ tới nó, ví dụ:
`C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\fxc.exe`
