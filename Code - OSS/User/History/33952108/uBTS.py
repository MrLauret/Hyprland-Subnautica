import torch
from torch import nn
from torch.utils.data import DataLoader, Dataset

eps = 1e-7
learning_rate = 0.1

torch.manual_seed(42)
X = torch.randn(50, 5, dtype=torch.float32)
Y = torch.randint(0, 4, (50,))

dataset = Dataset(inputs, targets)
dataloader = DataLoader(dataset, batch_size=64)

model = nn.Module(
    nn.Linear(5, 8),
    nn.ReLU(),
    nn.Linear(8, 4)
)

for epoch in range(10000):
    for inputs, targets in dataloader:
        logits = model(input)



prediciones = torch.argmax(P, dim=1)
aciertos = (prediciones == Y)
accuracy = aciertos.float().mean()

print(accuracy.item() * 100)