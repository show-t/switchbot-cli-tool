# switchbot-cli-tool

A command-line interface (CLI) tool written in Rust to control SwitchBot devices using the SwitchBot Web API (v1.1).  
SwitchBot Web API（v1.1）を使用して、SwitchBotデバイスを操作するための Rust 製コマンドラインツールです。

## Features

### 🔍 List Devices / デバイスの一覧表示: 
Retrieve all SwitchBot devices linked to your account, displaying details such as name, device ID, and device type.  
アカウントにリンクされているすべてのSwitchBotデバイスを取得し、名前・ID・種類などを表示します。

### 🎛️ Control Devices: 
Send commands to supported devices directly from your terminal.  
The following command types are available:  
対応デバイスに対して、ターミナルからコマンドを直接送信できます。対応しているコマンドは以下のとおりです：
- `TurnOn` / `TurnOff`:  
    Power devices on or off  
    デバイスの電源をオン／オフにします

- `SetBrightness`:  
    Set brightness level (numeric value)  
    明るさを数値で設定します

- `SetColor`:  
    Specify RGB color values  
    RGB値で色を指定します

- `SetColorTemperature`:  
    Set color temperature  
    色温度を設定します

- `AcSetAll`:  
    Set air conditioner temperature, mode, fan speed, etc., all at once  
    エアコンの温度、モード、風量などを一括設定します

- `Custom`:  
    Send custom commands by specifying `commandType`, `command`, and `parameter` manually  
    `commandType`・`command`・`parameter`を手動で指定し、カスタムコマンドを送信できます

## Requirements / 必要要件

- **Rust 1.75 or higher (stable)**    
  Requires a stable Rust toolchain version 1.75 or later.  
  本ツールはRust製で、バージョン1.75以上の安定版が必要です。

- **A SwitchBot account with cloud service enabled**  
  You must enable cloud service in the SwitchBot app settings.  
  SwitchBotアプリでクラウドサービスを有効にしておく必要があります。  
  

- **SwitchBot Open API token**  
  You can obtain the API token from the app under "Profile".  
  APIトークンはSwitchBotアプリの「プロフィール」から取得できます。  
  

## Installation / インストール

### 🧪 Build from source / ソースからビルド
Clone the repository and build the project:  
Cargo を使ってリポジトリをクローンし、手動でビルドしてください：

```bash
$ git clone https://github.com/show-t/switchbot-cli-tool.git
$ cd switchbot-cli-tool
$ cargo build --release
```

The compiled binary will be located at:  
ビルド後の実行ファイルは以下の場所に生成されます：

```bash
target/release/switchbot-cli
```

Optionally, move the binary to a directory in your PATH:  
必要に応じて、バイナリをPATHの通ったディレクトリに移動してください：

```sh
$ mv target/release/switchbot-cli /usr/local/bin/
```

### 🔐 Environment Variables / 環境変数の設定
Create a `.env` file in the project root and add your SwitchBot token:  
プロジェクトのルートに .env ファイルを作成し、SwitchBot のトークンを設定してください：
```env
SWITCHBOT_API_HOST='https://api.switch-bot.com/v1.1'
SWITCHBOT_TOKEN=<your_token_here>
SWITCHBOT_SECRET=<your_secret_here>

RUST_LOG=<log_level>
```

## Usage / 使い方

### 🔍 List devices / デバイスの一覧表示

```sh
$ switchbot-cli-tool list
```

### 🎛️ Control a device / デバイスを操作する

- You can control a device by specifying its device ID or alias.
デバイスIDまたはエイリアスを指定して操作できます。
```sh
$ switchbot-cli exec \
  --device livingroom-light \
  --command on
```

- You can also specify parameters using --values:
--values オプションでパラメータを渡すこともできます：
```sh
switchbot-cli-tool exec \
  --device <your-device-id> \
  --command set_brightness \
  --values 128
```

#### 🛠 Supported commands / 対応コマンド一覧  
- on / off
- set_brightness `<1-100>`
- color `<r:0-255>` `<g:0-255>` `<b:0-255>`
- color_temp `<2700-6500>`
- ac `<temperature>` `<mode:1-5>` `<fan_ speed:1-4>` `<power_state:on/off>`

#### 📁 Aliases / エイリアス設定

To use aliases, create a device_aliases.json file in the working directory with the following format:
エイリアスを使用するには、作業ディレクトリに以下の形式の device_aliases.json ファイルを作成してください。  

Each key is the alias name, and each value is the actual device ID.
キーがエイリアス名、値が実際のデバイスIDです。
```json
{
  "livingroom-light": "01-xxxxxxxxxxxx-yyyyyyyy",
  "bed_light": "02-xxxxxxxxxxxx-yyyyyyyy"
}
```

#### 🔹 Note / 補足:
Use `switchbot-cli --help` to explore all available options.  
すべてのオプションは `switchbot-cli --help` で確認できます。


## License / ライセンス

This project is licensed under the MIT License.  
本プロジェクトは MIT ライセンスのもとで公開されています。

See the [LICENSE](./LICENSE) file for more details.  
詳細は [LICENSE](./LICENSE) ファイルをご覧ください。


## Author / 作者

Created by [show-t](https://github.com/show-t)