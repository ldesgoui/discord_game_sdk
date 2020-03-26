#!/bin/sh

http \
    "https://discordapp.com/api/v6/applications/$DISCORD_CLIENT_ID/achievements" \
    "Authorization: Bot $DISCORD_BOT_TOKEN" \
    name:='{ "default": "test achievement" }' \
    description:='{ "default": "this is a test achievement" }' \
    secret:=false \
    secure:=false \
    icon='data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7'
