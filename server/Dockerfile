FROM python:3.13-slim-bullseye

# Set working directory
WORKDIR /app

# Install dependencies
COPY requirements.txt .
RUN pip install --upgrade pip && pip install -r requirements.txt

# Copy the application code
COPY . .

# Expose ports
EXPOSE 8000

# Run program
CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "8000"]
