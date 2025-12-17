# r34-hooks

> **Warning**  
> The associated web page `rule34.xxx` does show explicit and sexual content! Viewer discretion is advised.

A little application to crawl rule34.xxx for new images of specified tags and distribute them to different targets like Discord webhooks.

## Deployment

You can simply use the following Docker image for easy deployment.

```
ghcr.io/zekrotja/r34-hooks
```

```yaml
services:
  r34-hooks:
    image: ghcr.io/zekrotja/r34-hooks
    restart: unless-stopped
    environment:
      R34_STORAGE_DIR: /etc/r34-hooks/storage.json
      R34_LOG_LEVEL: info
      R34_SCHEDULE: "0 0 */12 * * *"
      R34_DYNAMIC_CONFIG_PATH: /etc/r34-hooks/dynamic-config.yaml
    volumes:
      - ./r34-hooks:/etc/r34-hooks
```

### Static Config

The static config is loaded on startup. It can either be passed as config file by passing the path to the file as first command argument or by environment variables. When you pass the static config via environment parameters, you need to specify a path to the dynamic config. Otherwise, the path of the static config will be used for the dynamic config, so that you can store the static and dynamic config together in one single file. The static config file can be passed in YAML or TOML format.

| Key                   | Type     | Required                                          | Description                                          |
| --------------------- | -------- | ------------------------------------------------- | ---------------------------------------------------- |
| `storage_dir`         | `string` | No (default: `storage.json`)                      | Storage file location                                |
| `log_level`           | `string` | No (default: `info`)                              | Log level                                            |
| `schedule`            | `string` | No                                                | Cron schedule (including seconds) for scheduled mode |
| `dynamic_config_path` | `string` | Only if config is passed as environment variables | Path to the dynamic config file                      |

### Dynamic Config

The dynamic config is loaded on every execution of the runner (when in scheduled mode). The dynamic config file can be passed in YAML or TOML format. Below you can find a quick example of the dynamic config.

```yaml
user_id: "123456"
api_token: "secrettokenfoobarbaz"

default_tags:
  - high_res
  - -ai_generated

targets:
  - tags:
      - kindred
    hook:
      discord:
        webhook_url: https://discord.com/api/webhooks/1234567890/webhooktokenexample
```

| Key            | Type       | Required            | Description                                                                                    |
| -------------- | ---------- | ------------------- | ---------------------------------------------------------------------------------------------- |
| `user_id`      | `string`   | Yes                 | Rule34 API user ID (see [options page](https://rule34.xxx/index.php?page=account&s=options))   |
| `api_token`    | `string`   | Yes                 | Rule34 API API token (see [options page](https://rule34.xxx/index.php?page=account&s=options)) |
| `default_tags` | `string[]` | No                  | Default tags which are and-linked to the target tags                                           |
| `targets`      | `Target[]` | Yes                 | List of targets                                                                                |
| `limit`        | `number`   | No (default: `100`) | Maximum number of requested entries per listing request                                        |

#### `Target`

| Key    | Type       | Required                           | Description                                 |
| ------ | ---------- | ---------------------------------- | ------------------------------------------- |
| `tags` | `string[]` | Yes                                | List of and-linked tags                     |
| `hook` | `Hook`     | Yes                                | Hook configuration                          |
| `id`   | `string`   | No (default: `tags` joiend by `,`) | The ID of the target (relevant for storage) |

#### `Hook`

##### `Discord`

| Key           | Type     | Required | Description                |
| ------------- | -------- | -------- | -------------------------- |
| `webhook_url` | `string` | Yes      | URL of the discord webhook |
