import asyncio
import string_sum
import logging

log = logging.getLogger("main.py")

log.info(string_sum.sum_as_string(20, 5))


async def main():
    try:
        string_sum.my_function2()
    except string_sum.MyError as e:
        log.error(e)

    log.info(await string_sum.sum_as_string_but_slow(5, 20))

    data = string_sum.MyPersistentData(5)

    log.info(data.counter)
    data.counter += 1
    log.info(data.counter)

    log.info(await data.add_to_counter(4))

    # this should log data
    string_sum.my_function3(data)
    data.counter += 1
    string_sum.my_function3(data)

if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    asyncio.run(main())
