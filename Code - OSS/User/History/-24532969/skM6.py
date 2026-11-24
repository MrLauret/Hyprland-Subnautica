import torch
import torch.nn as nn
from torch.utils.data import TensorDataset, DataLoader
import pandas as pd

def main() -> None:
    table = pd.pandas.read_csv("mnist_test.csv")
    labels = table.iloc[:, 0]
    pixels = table.iloc[:, 1:] / 255.0

    pixels_tensor = torch.tensor(pixels.values, dtype=torch.float32)
    labels_tensor = torch.tensor(labels.values, dtype=torch.long)

    dataset = TensorDataset(pixels_tensor, labels_tensor)
    loader = DataLoader(dataset, batch_size=64, shuffle=True)

    model = nn.Sequential(
        nn.Linear(784, 128),
        nn.ReLU(),
        nn.Linear(128, 64),
        nn.ReLU(),
        nn.Linear(64, 10),
    )

    loss_fn = nn.CrossEntropyLoss()
    optimizer = torch.optim.Adam(model.parameters(), lr=0.01)

    model.train()
    for epoch in range(30):

        total_loss = 0.0

        for batch_pixels, batch_labels in loader:

            outputs = model(batch_pixels)

            loss = loss_fn(outputs, batch_labels)

            optimizer.zero_grad()
            
            loss.backward()

            optimizer.step()

        if epoch % 5 == 0:
            loss_str = format(loss.item(), '.25f')
            print(f"\nEpoch: {epoch}\nLoss: {loss_str}")
    
    with torch.no_grad():
        model.eval()
        outputs = model(pixels_tensor)
        predictions = outputs.argmax(dim=1)

        print("\nPrimeras 5 predicciones del modelo:\n", predictions[10:15])
        print("\nPrimeras 5 etiquetas reales:\n", labels_tensor[10:15])


if __name__ == "__main__":
    main()