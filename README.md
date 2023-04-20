# r34-hooks

> **Warning**  
> The associated web page `rule34.xxx` does show explicit and sexual content! Viewer discretion is advised.

This (maybe a little bit over engineered) serverless application checks rule34.xxx for new contents for a given tag combination and sends new entries to specified hook targets.

## How does it work?

In the crate `handlers` under `api/invoke.rs`, you can find a Vercel serverless handler which will on invocation create a new instance of `Scraper` with persistence and `watcher` configuration extracted from the set environment variables.

As persistence layer only PostgreSQL is currently implemented.

A `watcher` consists of an `Uid`, a collection of image tags and a hook implementation instance. When the handler gets invoked, the following procedure is executed for each registered `watcher` entry.

First, an entry for `last_id` will be looked up for the `watcher` Uid in the persistence layer (i.e. Postgres database). If no entry is available or if the image behind the Uid has been deleted, the latest image for the tag combination will be requested and the ID of the image will be stored with the `watcher` Uid. After that, the execution terminates.

If a value for `last_id` has been recovered and the image still exists, the API will fetch new images for the given tag combination pagewise until the image with the UID has been found.

The collection of new imsages is then sent via the specified hook implementation (i.e. Discord web hooks).

## Deployment

To deploy the Vercel serverless function, you first need to install the [Vercel CLI](https://vercel.com/docs/cli) and log in with your account.

Clone the repository and deploy the app to Vercel afterwards.
```sh
git clone https://github.com/zekrotja/r34-hooks.git
cd r34-hooks
vercel --prod
```

After that, configure the project via environment variables using the Vercel CLI.

First, specify the Postgres database URL.
```sh
echo "postgresql://user:password@address:port/db" \
    | vercel env add R34_DATABASE_POSTGRES production
```

After that, we want to define our watchers. First, define the image tags to be watched (separated by `,`).
```sh
echo "kindred,-futanari" \
    | vercel env add R34_WATCH_myguild1_TAGS production
```

Now, we need to define the hook used for the watcher.
```sh
echo "https://discord.com/api/webhooks/<channel_id>/<token>" \
    | vercel env add R34_WATCH_myguild1_HOOK_DISCORD production
```

You can specify mutliple watcher configuration by defining the previous mentioned variables using different UIDs.
```
R34_WATCH_<uid>_TAGS
R34_WATCH_<uid>_HOOK_<hook_impl>
```

Examples:
```
R34_WATCH_guild1kindred_TAGS
R34_WATCH_guild1kindred_HOOK_DISCORD

R34_WATCH_guild1kaisa_TAGS
R34_WATCH_guild1kaisa_HOOK_DISCORD
```

After that you might need to re-deploy the production deployment of the project to apply the environment configuration.

Then, simply call the serverless function route to invoke the configured scraper.
```
GET <vercel_url>/api/invoke
```

The configuration also provides a cronjob which calls the `invoke` handler every day.