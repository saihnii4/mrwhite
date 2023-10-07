import nextcord
import nextcord.ext.commands

import typing

def complementary(type, id):
    def pred(ctx: nextcord.ext.commands.Context):
        print("bruh")
        if (not hasattr(ctx, type)):
            raise KeyError("bruh")
        return getattr(ctx, type).id == id
    return nextcord.ext.commands.check(pred)
        

class Lectures(nextcord.ext.commands.Cog):
    def __init__(self, bot: nextcord.ext.commands.Bot):
        self.bot = bot

    async def setup(self):
        ch: nextcord.TextChannel = self.bot.get_channel(1160076257459970129)
        if ch is None: pass
        self.lectures = await ch.history(oldest_first=True).flatten()

    @nextcord.ext.commands.Cog.listener()
    @complementary("channel", 1160076257459970129)
    async def on_message(self, msg: nextcord.Message):
        print("Testing")
        self.lectures = await msg.channel.history(oldest_first=True).flatten()

    @nextcord.ext.commands.Cog.listener()
    async def on_error(self, err):
        await ctx.reply(err)

    @nextcord.ext.commands.command()
    @complementary("channel", 1160076257459970129)
    async def test(self, ctx: nextcord.ext.commands.Context):
        await ctx.send("bruh")