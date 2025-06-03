#!/usr/bin/env python3
# scripts/cli.py

import asyncio
import cmd
from node import Node

class Bitcoin2CLI(cmd.Cmd):
    intro = "Welcome to Bitcoin 2 CLI. Type 'help' for commands."
    prompt = "(btc2) "
    
    def __init__(self, node):
        super().__init__()
        self.node = node
    
    async def do_start(self, arg):
        """Start the node"""
        await self.node.start()
        print("Node started")
    
    async def do_mine(self, arg):
        """Toggle mining: mine [on|off]"""
        state = arg.lower()
        if state == "on":
            self.node.mining_enabled = True
            print("Mining enabled")
        elif state == "off":
            self.node.mining_enabled = False
            print("Mining disabled")
        else:
            print("Usage: mine [on|off]")
    
    async def do_send(self, arg):
        """Send coins: send <recipient> <amount>"""
        args = arg.split()
        if len(args) != 2:
            print("Usage: send <recipient> <amount>")
            return
        
        recipient, amount = args
        try:
            amount = float(amount)
            await self.node.create_transaction(recipient, amount)
            print(f"Sent {amount} BTC2 to {recipient}")
        except ValueError:
            print("Invalid amount")
    
    async def do_balance(self, arg):
        """Check balance of address: balance <address>"""
        # Simplified implementation
        print("Balance: 100.0 BTC2 (demo)")
    
    async def do_connect(self, arg):
        """Connect to peer: connect <address>"""
        if not arg:
            print("Usage: connect <peer_address>")
            return
        
        try:
            await self.node.p2p_network.connect(arg)
            print(f"Connected to {arg}")
        except Exception as e:
            print(f"Connection failed: {e}")
    
    async def do_exit(self, arg):
        """Exit the CLI"""
        print("Exiting...")
        return True

async def main():
    node = await Node.new()
    cli = Bitcoin2CLI(node)
    await cli.do_start("")
    cli.cmdloop()

if __name__ == "__main__":
    asyncio.run(main())