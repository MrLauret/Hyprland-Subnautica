import torch
from torch import nn
from torch.utils.data import DataLoader, Dataset

eps = 1e-7
learning_rate = 0.01

torch.manual_seed(42)
X = torch.randn(50, 5, dtype=torch.float32)
Y = torch.randint(0, 4, (50,))

dataset = Dataset(inputs, targets)
dataloader = DataLoader(dataset, batch_size=64)

class NeuralNet(nn.Module):
    def __init__(self):
        super.__init__()
        self.hl = nn.Linear(5, 8)
        self.relu = nn.ReLU()
        self.ol = nn.Linear(8, 4)

model = NeuralNet()

optimizer = torch.optim.SGD(model.parameters(), learning_rate)
loss_fn = nn.CrossEntropyLoss()

for epoch in range(10000):
    for inputs, targets in dataloader:
        logits = model(input)

        nn.CrossEntropyLoss()

        optimizer.step()
        optimizer.zero_grad()


prediciones = torch.argmax(model(X), dim=1)
aciertos = (prediciones == Y)
accuracy = aciertos.float().mean()

print(accuracy.item() * 100)