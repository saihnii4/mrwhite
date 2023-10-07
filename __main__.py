#!/usr/bin/python

import nextcord
from cogs import cogs
import dotenv
import nextcord.ext.commands

import os
import logging

dotenv.load_dotenv()

class Bot(nextcord.ext.commands.Bot):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.load_extension("onami")
        return
    
    async def on_ready(self):
        logging.info("|| Yo, Mr. White, we're cooking yo. ||")
        for _cog in cogs:
            cog = _cog(self)
            print("|| Loading %s ||" % (cog.__cog_name__))
            self.add_cog(cog)
            await cog.setup()

    
bot = Bot(command_prefix='!', intents=nextcord.Intents.all())
bot.run(os.environ.get("TOKEN"))