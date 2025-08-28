_SHOULD_IGNORE = 0

import requests
import tkinter as tk
from tkinter import messagebox

def get_crypto_prices():
    url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum,solana&vs_currencies=usd"

    try:
        response = requests.get(url)
        response.raise_for_status()
        data = response.json()

        btc_price = data.get('bitcoin', {}).get('usd', 'N/A')
        eth_price = data.get('ethereum', {}).get('usd', 'N/A')
        sol_price = data.get('solana', {}).get('usd', 'N/A')

        message = (f"BTC current price: {btc_price} "
                   f"\nETH current price: {eth_price} "
                   f"\nSOL current price: {sol_price} ")

        root = tk.Tk()
        root.withdraw()
        messagebox.showinfo("Crypto Prices", message)

    except requests.exceptions.RequestException as e:
        tk.Tk().withdraw()
        messagebox.showerror("Error getting prices from coingecko:", e)

def on_break_reminder(_context):
    get_crypto_prices()

# if __name__ == "__main__":
#     get_crypto_prices()