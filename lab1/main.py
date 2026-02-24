#!/usr/bin/env python
# coding: utf-8

import typing
import pickle
import os
import pathlib
import datetime
import numpy as np
import matplotlib.pyplot as plt
import scipy.stats

import pr_classes

def main():
    # Налаштування шляхів
    SCRIPT_PATH = pathlib.Path(os.path.abspath(__file__)) # або вкажіть ваш шлях явно
    SCRIPT_DIR = SCRIPT_PATH.parent
    BUILD_DIR = SCRIPT_DIR / 'build'
    PLOTS_DIR = BUILD_DIR / 'plots'
    
    # Створюємо папку для графіків, якщо її немає
    PLOTS_DIR.mkdir(parents=True, exist_ok=True)

    # Завантаження даних
    with open(BUILD_DIR / 'ser_prs.pickle', 'rb') as fileio:
        ser_prs = typing.cast(pr_classes.PrBlob, pickle.load(fileio))

    # =========================================================================
    # Графік 1: Розмір PR vs Щільність (Без фільтрів)
    # =========================================================================
    size_list_unfiltered: list[int] = []
    density_list_unfiltered: list[float] = []

    for pr in ser_prs.prs:
        size = pr.additions + pr.deletions
        if size == 0:
            continue
        density = len(pr.comments) / size
        size_list_unfiltered.append(size)
        density_list_unfiltered.append(density)

    plt.figure(figsize=(10, 6))
    plt.scatter(size_list_unfiltered, density_list_unfiltered, alpha=0.5)
    plt.title('Залежність: Розмір PR vs Щільність коментарів (Усі дані)')
    plt.xlabel('Розмір PR (додані + видалені рядки)')
    plt.ylabel('Щільність коментарів (коментарі / рядки)')
    plt.grid(True, linestyle='--', alpha=0.7)
    
    plt.tight_layout()
    plt.savefig(PLOTS_DIR / '1_size_vs_density_unfiltered.svg')
    plt.close()

    # =========================================================================
    # Графік 2: Розмір PR vs Щільність (Відфільтровано до 1000 рядків)
    # =========================================================================
    size_list: list[int] = []
    density_list: list[float] = []

    for pr in ser_prs.prs:
        size = pr.additions + pr.deletions
        if size == 0 or size > 1000:
            continue
        density = len(pr.comments) / size
        size_list.append(size)
        density_list.append(density)

    corr_size_density, p_value1 = scipy.stats.pearsonr(size_list, density_list)
    print(f"Кореляція (Розмір vs Щільність): {corr_size_density:.4f} (p-value: {p_value1:.4e})")

    plt.figure(figsize=(10, 6))
    plt.scatter(size_list, density_list, alpha=0.5)
    plt.title('Залежність: Розмір PR (до 1000 рядків) vs Щільність коментарів')
    plt.xlabel('Розмір PR (додані + видалені рядки)')
    plt.ylabel('Щільність коментарів (коментарі / рядки)')
    plt.grid(True, linestyle='--', alpha=0.7)
    
    plt.tight_layout()
    plt.savefig(PLOTS_DIR / '2_size_vs_density_filtered.svg')
    plt.close()

    # =========================================================================
    # Графік 3: Кількість змінених файлів vs Кількість коментарів
    # =========================================================================
    files_list: list[int] = []
    comments_count_list: list[int] = []

    for pr in ser_prs.prs:
        if pr.changedFiles < 100:
            files_list.append(pr.changedFiles)
            comments_count_list.append(len(pr.comments))

    corr_files_comments, p_value2 = scipy.stats.pearsonr(files_list, comments_count_list)
    print(f"Кореляція (Файли vs Коментарі): {corr_files_comments:.4f} (p-value: {p_value2:.4e})")

    plt.figure(figsize=(10, 6))
    plt.scatter(files_list, comments_count_list, alpha=0.5, color='orange')
    plt.title('Кореляція: Кількість змінених файлів vs Кількість коментарів')
    plt.xlabel('Кількість змінених файлів')
    plt.ylabel('Загальна кількість коментарів')
    plt.grid(True, linestyle='--', alpha=0.7)
    
    plt.tight_layout()
    plt.savefig(PLOTS_DIR / '3_files_vs_comments.svg')
    plt.close()

    # =========================================================================
    # Графік 4: Довжина опису PR vs Кількість коментарів
    # =========================================================================
    pr_body_lens: list[int] = []
    pr_comments_counts: list[int] = []
    
    for pr in ser_prs.prs:
        body_text = pr.body if hasattr(pr, 'body') and pr.body else ""
        body_len = len(body_text)
        if body_len < 4000:
            pr_body_lens.append(body_len)
            pr_comments_counts.append(len(pr.comments))

    plt.figure(figsize=(10, 6))
    plt.scatter(pr_body_lens, pr_comments_counts, alpha=0.5, color='orange')
    plt.title('Залежність: Довжина опису PR vs Кількість коментарів')
    plt.xlabel('Довжина опису PR (кількість символів)')
    plt.ylabel('Кількість коментарів')
    plt.grid(True, linestyle='--', alpha=0.7)
    
    plt.tight_layout()
    plt.savefig(PLOTS_DIR / '4_body_len_vs_comment_count.svg')
    plt.close()

    # =========================================================================
    # Графік 5: Довжина опису PR vs Загальний обсяг дискусії
    # =========================================================================
    pr_body_lens_total: list[int] = []
    pr_comments_len_total: list[int] = []

    for pr in ser_prs.prs:
        body_text = pr.body if hasattr(pr, 'body') and pr.body else ""
        body_len = len(body_text)
        if body_len < 4000:
            pr_body_lens_total.append(body_len)
            # Сумуємо довжину тексту всіх коментарів (з перевіркою на None)
            total_len = sum(len(c.body) for c in pr.comments if hasattr(c, 'body') and c.body)
            pr_comments_len_total.append(total_len)

    plt.figure(figsize=(10, 6))
    plt.scatter(pr_body_lens_total, pr_comments_len_total, alpha=0.5, color='orange')
    plt.title('Залежність: Довжина опису PR vs Загальний обсяг дискусії')
    plt.xlabel('Довжина опису PR (кількість символів)')
    plt.ylabel('Сумарна довжина коментарів (кількість символів)')
    plt.grid(True, linestyle='--', alpha=0.7)
    
    plt.tight_layout()
    plt.savefig(PLOTS_DIR / '5_body_len_vs_total_comment_len.svg')
    plt.close()

    # =========================================================================
    # Графік 6: Синдром Пінг-Понгу
    # =========================================================================
    class CommitEvent(typing.NamedTuple):
        dt: datetime.datetime

    class CommentEvent(typing.NamedTuple):
        dt: datetime.datetime

    type PrEvent = CommitEvent | CommentEvent

    ping_pong_counts: list[int] = []
    merge_times_days: list[float] = []

    for pr in ser_prs.prs:
        if pr.mergedAt is None:
            continue

        events: list[PrEvent] = []
        for c in pr.commits:
            events.append(CommitEvent(datetime.datetime.fromisoformat(c.committedDate.replace('Z', '+00:00'))))
        for c in pr.comments:
            if c.author.login != pr.author.login:
                events.append(CommentEvent(datetime.datetime.fromisoformat(c.createdAt.replace('Z', '+00:00'))))
        
        events.sort(key=lambda x: x.dt)

        first_comment_idx = -1
        for i, e in enumerate(events):
            if isinstance(e, CommentEvent):
                first_comment_idx = i
                break

        if first_comment_idx == -1:
            continue

        ping_pongs = 0
        prev_event = events[first_comment_idx]
        for e in events[first_comment_idx + 1:]:
            if e.__class__ != prev_event.__class__:
                ping_pongs += 1
                prev_event = e

        cycles = ping_pongs // 2

        created = datetime.datetime.fromisoformat(pr.createdAt.replace('Z', '+00:00'))
        merged = datetime.datetime.fromisoformat(pr.mergedAt.replace('Z', '+00:00'))
        duration_days = (merged - created).total_seconds() / (3600 * 24)

        if duration_days > 150:
            continue

        ping_pong_counts.append(cycles)
        merge_times_days.append(duration_days)

    plt.figure(figsize=(14, 6))
    plt.scatter(ping_pong_counts, merge_times_days, alpha=0.5, color='purple')
    plt.title('Синдром Пінг-Понгу: Цикли чергування vs Час до злиття')
    plt.xlabel('Кількість циклів Пінг-Понгу (Коментар <-> Коміт)')
    plt.ylabel('Час до злиття (дні)')
    plt.grid(True, linestyle='--', alpha=0.7)
    
    plt.tight_layout()
    plt.savefig(PLOTS_DIR / '6_ping_pong_syndrome.svg')
    plt.close()

if __name__ == '__main__':
    main()