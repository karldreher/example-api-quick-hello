FROM python:3.11-slim-bullseye
# TODO: Fix python-version to be 3.11.4, likewise here

COPY requirements.txt .
COPY main.py .
RUN pip install --no-cache-dir --upgrade -r requirements.txt
EXPOSE 8000
CMD ["uvicorn", "main:app"]
