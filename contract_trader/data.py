from pycoingecko import CoinGeckoAPI
from datetime import datetime
from pandas import DataFrame as df

cg = CoinGeckoAPI()

data = cg.get_coin_market_chart_by_id(id='polkadot', vs_currency='usd', days=350)
prices = data['prices']

print(f"Prices for Polkadot for the last {len(prices)} days:")

for timestamp, price in prices:
    # // convert ts to date in python
    timestamp = datetime.fromtimestamp(timestamp / 1000)
    print(f"Timestamp: {timestamp}, Price: {price}")


df = df(prices)

df[1] = (df[1] * pow(10, 2)).astype(int)

# print the first 5 rows
print(df.head())

df.to_csv('polkadot_prices.csv', header=False, index=False)
