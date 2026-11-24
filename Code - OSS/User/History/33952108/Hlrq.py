import torch
from torch import nn
from torch.utils.data import DataLoader, Dataset

eps = 1e-7
learning_rate = 0.1

torch.manual_seed(42)
inputs = torch.randn(50, 5, dtype=torch.float32)
targets = torch.randint(0, 4, (50,))

dataset = Dataset(inputs, targets)
dataloader = DataLoader(dataset)

model = nn.Module(
    nn.Linear(5, 8),
    nn.ReLU(),
    nn.Linear(8, 4)
)

for epoch in range(10000):
    for row, column in inputs:
        logits = model()



prediciones = torch.argmax(P, dim=1)
aciertos = (prediciones == Y)
accuracy = aciertos.float().mean()

print(accuracy.item() * 100)